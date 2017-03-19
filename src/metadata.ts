import * as Joi from 'joi';

export interface Calendar {
  year?: number;
  month?: number;
  week?: number;
}

export interface Chip extends Calendar {
  type?: string;
  label: string | null;
  manufacturer?: string;
  outlier?: boolean;
}

export namespace Chip {
  export const schema = Joi.object().keys({
    type: Joi.string(),
    label: Joi.string().allow(null).required(),
    manufacturer: Joi.string(),
    year: Joi.number(),
    month: Joi.number(),
    week: Joi.number(),
    outlier: Joi.boolean()
  });
}

export interface Metadata {
  serial: string;
  type: string;
}

export interface DmgMetadata extends Metadata {
  type: "DMG";
  mainboard: {
    type: string;
    cpu?: Chip;
  };
}

export namespace DmgMetadata {
  export const schema = Joi.object().keys({
    type: Joi.string().required(),
    mainboard: Joi.object().required().keys({
      type: Joi.string().required(),
      cpu: Chip.schema,
    })
  });
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

export interface MgbMetadata extends Metadata {
  type: "MGB";
  mainboard: {
    type: string;
    cpu?: Chip;
  };
}

export namespace MgbMetadata {
  export const schema = Joi.object().keys({
    type: Joi.string().required(),
    mainboard: Joi.object().required().keys({
      type: Joi.string().required(),
      cpu: Chip.schema,
    })
  });
}

export interface MglMetadata extends Metadata {
  type: "MGL";
  mainboard: {
    type: string;
    cpu?: Chip;
  };
}

export namespace MglMetadata {
  export const schema = Joi.object().keys({
    type: Joi.string().required(),
    mainboard: Joi.object().required().keys({
      type: Joi.string().required(),
      cpu: Chip.schema,
    })
  });
}

export interface Sgb2Metadata extends Metadata {
  type: "SGB2";
  stamp?: string;
  mainboard: {
    type: string;
    circled_letters?: string;
    letter_at_top_right?: string;
    crystal?: Chip;
    cpu?: Chip;
    icd2?: Chip;
    work_ram?: Chip;
    rom?: Chip;
    cic?: Chip;
    coil?: Chip;
    year?: number;
    month?: number;
  };
}

export namespace Sgb2Metadata {
  export const schema = Joi.object().keys({
    type: Joi.string().required(),
    stamp: Joi.string(),
    mainboard: Joi.object().required().keys({
      type: Joi.string().required(),
      circled_letters: Joi.string(),
      letter_at_top_right: Joi.string(),
      crystal: Chip.schema,
      cpu: Chip.schema,
      icd2: Chip.schema,
      work_ram: Chip.schema,
      rom: Chip.schema,
      cic: Chip.schema,
      coil: Chip.schema,
      year: Joi.number(),
      month: Joi.number()
    })
  });
}

export interface CgbMetadata extends Metadata {
  type: "CGB";
  mainboard: {
    type: string;
    cpu?: Chip;
  };
}

export namespace CgbMetadata {
  export const schema = Joi.object().keys({
    type: Joi.string().required(),
    mainboard: Joi.object().required().keys({
      type: Joi.string().required(),
      cpu: Chip.schema,
    })
  });
}

export interface AgbMetadata extends Metadata {
  type: "AGB";
  color?: string;
  mainboard: {
    type: string;
    number_pair?: string;
    stamp?: string;
    circled_letters?: string;
    crystal?: Chip;
    cpu?: Chip;
    work_ram?: Chip;
    regulator?: Chip | null;
    amplifier?: Chip;
    u4?: Chip;
    year?: number;
    month?: number;
  };
}

export namespace AgbMetadata {
  export const schema = Joi.object().keys({
    type: Joi.string().required(),
    color: Joi.string(),
    mainboard: Joi.object().required().keys({
      type: Joi.string().required(),
      number_pair: Joi.string(),
      stamp: Joi.string(),
      circled_letters: Joi.string(),
      crystal: Chip.schema,
      cpu: Chip.schema,
      work_ram: Chip.schema,
      regulator: Chip.schema.allow(null),
      amplifier: Chip.schema,
      u4: Chip.schema,
      year: Joi.number(),
      month: Joi.number()
    })
  });
}

export interface AgsMetadata extends Metadata {
  type: "AGS";
  mainboard: {
    type: string;
    cpu?: Chip;
  };
}

export namespace AgsMetadata {
  export const schema = Joi.object().keys({
    type: Joi.string().required(),
    mainboard: Joi.object().required().keys({
      type: Joi.string().required(),
      cpu: Chip.schema,
    })
  });
}

export interface GbsMetadata extends Metadata {
  type: "GBS";
  color?: string;
  mainboard: {
    type: string;
    number_pair?: string,
    stamp?: string;
    stamp_front?: string;
    stamp_back?: string;
    circled_letters?: string;
    year?: number;
    month?: number;
    crystal?: Chip;
    cpu?: Chip;
    work_ram?: Chip;
    u4?: Chip;
    u5?: Chip;
    u6?: Chip;
  };
}

export namespace GbsMetadata {
  export const schema = Joi.object().keys({
    type: Joi.string().required(),
    color: Joi.string(),
    mainboard: Joi.object().required().keys({
      type: Joi.string().required(),
      number_pair: Joi.string(),
      stamp: Joi.string(),
      stamp_front: Joi.string(),
      stamp_back: Joi.string(),
      circled_letters: Joi.string(),
      year: Joi.number(),
      month: Joi.number(),
      crystal: Chip.schema,
      cpu: Chip.schema,
      work_ram: Chip.schema,
      u4: Chip.schema,
      u5: Chip.schema,
      u6: Chip.schema
    })
  });
}

export interface OxyMetadata extends Metadata {
  type: "OXY";
  color?: string;
  mainboard: {
    type: string;
    circled_letters?: string;
    cpu?: Chip;
    u2?: Chip;
    u4?: Chip;
    u5?: Chip;
    year?: number;
    month?: number;
  };
}

export namespace OxyMetadata {
  export const schema = Joi.object().keys({
    type: Joi.string().required(),
    color: Joi.string(),
    mainboard: Joi.object().required().keys({
      type: Joi.string().required(),
      circled_letters: Joi.string(),
      cpu: Chip.schema,
      u2: Chip.schema,
      u4: Chip.schema,
      u5: Chip.schema,
      year: Joi.number(),
      month: Joi.number()
    })
  });
}
