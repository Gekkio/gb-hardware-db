import * as Joi from 'joi';

export interface Chip {
  type: string;
  label: string;
  manufacturer?: string;
  year?: number;
  week?: number;
  outlier?: boolean;
}

export namespace Chip {
  export const schema = Joi.object().keys({
    type: Joi.string().required(),
    label: Joi.string().required(),
    manufacturer: Joi.string(),
    year: Joi.number(),
    week: Joi.number(),
    outlier: Joi.boolean()
  });
}

export interface Metadata {
  serial: string;
  type: string;
}

export interface SgbMetadata extends Metadata {
  type: "SGB";
  stamp?: string;
  mainboard: {
    type: string;
    circled_letters?: string;
    letter_at_top_right?: string;
    cpu?: Chip;
    icd2?: Chip;
    work_ram?: Chip;
    video_ram?: Chip;
    rom?: Chip;
    cic?: Chip;
    year?: number;
    month?: number;
  };
}

export namespace SgbMetadata {
  export const schema = Joi.object().keys({
    type: Joi.string().required(),
    stamp: Joi.string(),
    mainboard: Joi.object().required().keys({
      type: Joi.string().required(),
      circled_letters: Joi.string(),
      letter_at_top_right: Joi.string(),
      cpu: Chip.schema,
      icd2: Chip.schema,
      work_ram: Chip.schema,
      video_ram: Chip.schema,
      rom: Chip.schema,
      cic: Chip.schema,
      year: Joi.number(),
      month: Joi.number()
    })
  });
}
